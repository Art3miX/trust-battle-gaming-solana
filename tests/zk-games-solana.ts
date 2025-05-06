import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import {ZkGamesSolana} from "../target/types/zk_games_solana";
import {Keypair, LAMPORTS_PER_SOL, PublicKey} from "@solana/web3.js";
import * as bip39 from "bip39";
import {assert} from "chai";

const TEST_SEED =
  "vague parrot cook twelve fan flush curve web coffee pet angry mammal";
const TEST_PUB = "3xoJZkhxuzKpKATL7UhskTA17uBuEnMeuLAqhovETHg4";
const PLAYER1_USERNAME = "player1";
const PLAYER2_USERNAME = "player2";

describe("zk-games-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.zkGamesSolana as Program<ZkGamesSolana>;
  const admin_seed = bip39.mnemonicToSeedSync(TEST_SEED, "1234");
  const admin = Keypair.fromSeed(admin_seed.subarray(0, 32));
  const gameClient = new Keypair();

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
        loginHash: Array(32).fill(0),
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
    let gameId = new BN(1);

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

    let [player2RpsBasicPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("rps_basic_player"), Buffer.from(PLAYER2_USERNAME)],
      program.programId
    );

    // Register player1 to rps_basic
    await program.methods
      .registerPlayerRpsBasic()
      .accounts({
        signer: gameClient.publicKey,
        player: player2Pda,
        gameClient: gameClientPda,
      })
      .signers([gameClient])
      .rpc();

    // Player1 starts a new game
    // TODO: Generate choice hash
    await program.methods
      .initRpsBasic({
        id: gameId,
        choiceHash: Array(32).fill(0),
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

    // Configrm gamePda was created
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

    // Player1 complete the game where player 1 wins
    await program.methods
      .completeRpsBasic({
        player1Choice: 1,
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
      .rpc();

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
