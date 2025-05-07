import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import {ZkGamesSolana} from "../target/types/zk_games_solana";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
} from "@solana/web3.js";
import * as bip39 from "bip39";
import {assert} from "chai";
import {createHash} from "crypto";

const TEST_SEED =
  "vague parrot cook twelve fan flush curve web coffee pet angry mammal";
const TEST_PUB = "3xoJZkhxuzKpKATL7UhskTA17uBuEnMeuLAqhovETHg4";
const PLAYER1_USERNAME = "player1";
const PLAYER2_USERNAME = "player2";

// Test secret for player1 and "1234"
const TEST_SECRET = [
  173, 240, 188, 129, 221, 80, 206, 211, 71, 132, 81, 173, 92, 229, 200, 234,
  55, 52, 184, 124, 212, 246, 121, 193, 100, 19, 218, 95, 99, 194, 82, 107,
];
// Test login hash for the above secret
const TEST_LOGIN_HASH = [
  71, 236, 31, 38, 239, 68, 192, 36, 80, 51, 192, 143, 249, 216, 6, 119, 129,
  152, 181, 144, 140, 225, 51, 127, 84, 202, 222, 235, 116, 153, 137, 44,
];

// Test secret for player2
const TEST_SECRET2 = [
  19, 238, 50, 236, 239, 88, 24, 182, 255, 126, 139, 62, 119, 140, 57, 174, 85,
  155, 66, 57, 100, 120, 241, 21, 211, 115, 67, 241, 238, 145, 90, 81,
];
// Test login hash for player2
const TEST_LOGIN_HASH2 = [
  229, 250, 123, 208, 56, 15, 168, 40, 245, 241, 72, 159, 159, 28, 170, 150, 20,
  13, 73, 28, 40, 172, 240, 205, 191, 163, 9, 6, 44, 175, 242, 173,
];

const PROOF_P1_G0_C1 = [
  17, 182, 160, 157, 43, 175, 136, 58, 48, 193, 74, 163, 254, 240, 246, 192,
  232, 39, 85, 193, 73, 210, 168, 248, 231, 91, 238, 41, 228, 107, 138, 41, 248,
  183, 221, 33, 40, 201, 41, 80, 175, 212, 139, 195, 225, 200, 119, 37, 139,
  248, 238, 166, 152, 68, 151, 54, 12, 24, 25, 164, 32, 79, 112, 126, 190, 1,
  18, 113, 27, 173, 111, 189, 251, 223, 196, 153, 156, 239, 185, 82, 18, 151,
  110, 122, 12, 171, 219, 80, 61, 15, 191, 118, 141, 186, 251, 138, 151, 169, 3,
  180, 26, 217, 220, 152, 97, 64, 177, 47, 73, 39, 50, 17, 6, 116, 3, 255, 239,
  198, 8, 113, 72, 18, 88, 154, 16, 89, 126, 2, 129, 49, 239, 143, 39, 164, 55,
  57, 186, 202, 83, 105, 18, 115, 9, 13, 47, 162, 169, 204, 213, 153, 153, 178,
  130, 77, 90, 70, 200, 210, 46, 82, 78, 213, 146, 94, 1, 232, 57, 187, 239, 52,
  129, 58, 236, 104, 77, 159, 2, 247, 128, 79, 178, 66, 116, 17, 206, 96, 149,
  194, 121, 195, 75, 66, 66, 32, 226, 45, 23, 11, 207, 76, 59, 95, 124, 65, 131,
  31, 183, 187, 152, 151, 220, 21, 147, 228, 137, 167, 187, 192, 45, 191, 58,
  147, 20, 56, 113, 193, 182, 97, 22, 35, 63, 7, 133, 212, 242, 104, 12, 242,
  93, 11, 201, 206, 144, 115, 47, 44, 85, 65, 180, 114, 137, 70, 255, 139, 142,
  5, 134, 204, 248, 141,
];

describe("zk-games-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.zkGamesSolana as Program<ZkGamesSolana>;
  const adminSeed = bip39.mnemonicToSeedSync(TEST_SEED, "1234");
  const admin = Keypair.fromSeed(adminSeed.subarray(0, 32));
  const gameClientSeed = bip39.mnemonicToSeedSync(TEST_SEED, "12345");
  const gameClient = Keypair.fromSeed(gameClientSeed.subarray(0, 32));

  let gameClientPda: PublicKey;
  let player1Pda: PublicKey;
  let player2Pda: PublicKey;

  before(async () => {
    await anchor
      .getProvider()
      .connection.requestAirdrop(admin.publicKey, LAMPORTS_PER_SOL * 1000);
    await anchor
      .getProvider()
      .connection.requestAirdrop(gameClient.publicKey, LAMPORTS_PER_SOL * 1000);

    // Increase max CU for proving

    // Add game client
    await program.methods
      .registerGameClient({
        name: "Test Game Client",
        signer: gameClient.publicKey,
      })
      .signers([admin])
      .rpc();

    // Get game client PDA
    gameClientPda = PublicKey.findProgramAddressSync(
      [Buffer.from("game_client"), gameClient.publicKey.toBuffer()],
      program.programId
    )[0];

    // Create player1
    await program.methods
      .registerPlayer({
        username: PLAYER1_USERNAME,
        loginHash: TEST_LOGIN_HASH,
      })
      .accounts({
        signer: gameClient.publicKey,
        gameClient: gameClientPda,
      })
      .signers([gameClient])
      .rpc();

    player1Pda = PublicKey.findProgramAddressSync(
      [Buffer.from("player"), Buffer.from(PLAYER1_USERNAME)],
      program.programId
    )[0];

    // Create player2
    await program.methods
      .registerPlayer({
        username: PLAYER2_USERNAME,
        loginHash: Array(32).fill(0),
      })
      .accounts({
        signer: gameClient.publicKey,
        gameClient: gameClientPda,
      })
      .signers([gameClient])
      .rpc();

    player2Pda = PublicKey.findProgramAddressSync(
      [Buffer.from("player"), Buffer.from(PLAYER2_USERNAME)],
      program.programId
    )[0];
  });

  it("Game client was created", async () => {
    let gameClientPdaBalance = await anchor
      .getProvider()
      .connection.getBalance(gameClientPda);

    assert(gameClientPdaBalance > 0, "Game client PDA balance is 0");

    let gameClientPdaData = await program.account.gameClient.fetch(
      gameClientPda
    );

    assert(
      gameClientPdaData.signer.toString() == gameClient.publicKey.toString(),
      "PDA signer is not gameClient pubkey"
    );
  });

  it("2 players were created", async () => {
    let player1PdaBalance = await anchor
      .getProvider()
      .connection.getBalance(player1Pda);

    assert(player1PdaBalance > 0, "Player1 PDA balance is 0");

    let player2PdaBalance = await anchor
      .getProvider()
      .connection.getBalance(player2Pda);

    assert(player2PdaBalance > 0, "Player2 PDA balance is 0");
  });

  it("Player RPS basic game", async () => {
    let gameId = new BN(0);
    let player1Choice = 1;

    // Register player1 to rps_basic
    await program.methods
      .registerPlayerRpsBasic()
      .accounts({
        signer: gameClient.publicKey,
        player: player1Pda,
        gameClient: gameClientPda,
      })
      .signers([gameClient])
      .rpc();

    let [player1RpsBasicPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("rps_basic_player"), Buffer.from(PLAYER1_USERNAME)],
      program.programId
    );

    // Register player2 to rps_basic
    await program.methods
      .registerPlayerRpsBasic()
      .accounts({
        signer: gameClient.publicKey,
        player: player2Pda,
        gameClient: gameClientPda,
      })
      .signers([gameClient])
      .rpc();

    let [player2RpsBasicPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("rps_basic_player"), Buffer.from(PLAYER2_USERNAME)],
      program.programId
    );

    // Player1 starts a new game
    // Generate choice hash
    let choice_hash = createHash("sha256")
      .update(Buffer.from(TEST_SECRET)) // TODO: get secret
      .update(gameClientPda.toString())
      .update(gameId.toString())
      .update(player1Choice.toString())
      .digest();

    await program.methods
      .initRpsBasic({
        id: gameId,
        choiceHash: Array.from(choice_hash),
      })
      .accounts({
        signer: gameClient.publicKey,
        player1: player1Pda,
        gameClient: gameClientPda,
      })
      .signers([gameClient])
      .rpc();

    let [gamePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("rps_basic_game"),
        gameClientPda.toBuffer(),
        gameId.toBuffer("le", 8),
      ],
      program.programId
    );

    // Confirm gamePda was created
    await program.account.rpsBasicGame.fetch(gamePda);

    // Player2 joins game
    await program.methods
      .joinRpsBasic({
        player2Choice: 0,
      })
      .accounts({
        signer: gameClient.publicKey,
        rpsBasicGame: gamePda,
        player1: player1Pda,
        player2: player2Pda,
        gameClient: gameClientPda,
      })
      .signers([gameClient])
      .rpc();

    // confirm player2 joined the game
    let gameData = await program.account.rpsBasicGame.fetch(gamePda);

    assert(gameData.player2, "Player2 did not join the game");

    // we need to increase our max CU for verification
    let ix = anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
      units: 400000,
    });
    // Player1 complete the game where player 1 wins
    let completeIx = await program.methods
      .completeRpsBasic({
        proof: Buffer.from(PROOF_P1_G0_C1),
        player1Choice,
      })
      .accounts({
        signer: gameClient.publicKey,
        rpsBasicGame: gamePda,
        player1: player1Pda,
        player1RpsBasicPda,
        player2: player2Pda,
        player2RpsBasicPda,
        gameClient: gameClientPda,
      })
      .signers([gameClient])
      .instruction();

    const tx = new Transaction().add(ix).add(completeIx);

    await sendAndConfirmTransaction(anchor.getProvider().connection, tx, [
      gameClient,
    ]);

    // Player 1 won, so player1RpsBasicPda should contain data about the win
    // and player2RpsBasicPda should contain data about the loss
    let player1RpsBasicData = await program.account.rpsBasicPlayer.fetch(
      player1RpsBasicPda
    );
    let player2RpsBasicData = await program.account.rpsBasicPlayer.fetch(
      player2RpsBasicPda
    );

    assert(
      player1RpsBasicData.totalGames.cmp(new BN(1)) === 0,
      "Player1 should have 1 game played"
    );
    assert(
      player2RpsBasicData.totalGames.cmp(new BN(1)) === 0,
      "Player2 should have 1 game played"
    );

    assert(
      player1RpsBasicData.totalWins.cmp(new BN(1)) === 0,
      "Player1 should have 1 game won"
    );
    assert(
      player2RpsBasicData.totalLosses.cmp(new BN(1)) === 0,
      "Player2 should have 1 game lost"
    );

    assert(
      player1RpsBasicData.totalChoices[1].cmp(new BN(1)) === 0,
      "Player1 chose choice 1"
    );
    assert(
      player2RpsBasicData.totalChoices[0].cmp(new BN(1)) === 0,
      "Player2 chose choice 0"
    );
  });
});
