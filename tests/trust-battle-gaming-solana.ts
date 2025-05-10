import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import {TrustBattleGamingSolana} from "../target/types/trust_battle_gaming_solana";
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
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

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
  68, 90, 177, 66, 217, 224, 89, 180, 231, 30, 113, 127, 192, 169, 222, 169,
  131, 220, 241, 199, 203, 39, 101, 47, 11, 37, 211, 129, 184, 56, 212, 103,
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
  17, 182, 160, 157, 22, 31, 19, 241, 147, 99, 0, 89, 142, 8, 247, 197, 187,
  124, 192, 4, 119, 231, 138, 222, 119, 164, 8, 43, 169, 183, 0, 16, 116, 45,
  41, 241, 3, 124, 115, 32, 234, 16, 208, 137, 198, 151, 182, 241, 149, 176,
  193, 244, 26, 200, 155, 230, 79, 244, 161, 76, 7, 23, 44, 215, 115, 225, 15,
  128, 30, 227, 176, 112, 246, 222, 222, 199, 92, 196, 125, 106, 184, 46, 162,
  232, 25, 97, 49, 137, 186, 185, 219, 162, 179, 236, 195, 79, 100, 95, 108, 98,
  23, 171, 73, 115, 76, 63, 28, 167, 23, 195, 60, 122, 52, 18, 140, 228, 37,
  108, 67, 104, 82, 29, 192, 242, 120, 133, 201, 57, 141, 79, 48, 198, 40, 222,
  118, 139, 6, 153, 82, 91, 236, 13, 77, 132, 226, 93, 188, 202, 7, 52, 162, 35,
  2, 48, 37, 193, 118, 133, 90, 84, 142, 242, 52, 200, 11, 153, 127, 156, 50,
  115, 116, 51, 181, 162, 117, 123, 124, 30, 177, 20, 249, 229, 196, 78, 67,
  232, 57, 227, 171, 24, 87, 237, 27, 218, 122, 20, 46, 137, 49, 147, 62, 154,
  151, 122, 235, 6, 63, 157, 151, 33, 9, 234, 151, 209, 86, 148, 152, 252, 33,
  147, 92, 82, 167, 150, 240, 190, 104, 80, 37, 194, 39, 227, 56, 14, 37, 157,
  17, 247, 101, 94, 179, 47, 69, 143, 83, 40, 2, 252, 23, 76, 54, 9, 254, 40,
  15, 226, 34, 212, 52, 44,
];

const MIN_AMOUNT = new BN(1_000_000);
const INIT_PLAYER_BAL = MIN_AMOUNT.mul(new BN(5));

describe("trust-battle-gaming-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .TrustBattleGamingSolana as Program<TrustBattleGamingSolana>;
  const adminSeed = bip39.mnemonicToSeedSync(TEST_SEED, "1234");
  const admin = Keypair.fromSeed(adminSeed.subarray(0, 32));
  const gameClientSeed = bip39.mnemonicToSeedSync(TEST_SEED, "12345");
  const gameClient = Keypair.fromSeed(gameClientSeed.subarray(0, 32));
  const platformAccSeed = bip39.mnemonicToSeedSync(TEST_SEED, "123456");
  const platformAcc = Keypair.fromSeed(platformAccSeed.subarray(0, 32));

  let managerPda: PublicKey;
  let vault: PublicKey;
  let usdcMint: PublicKey;

  let gameClientPda: PublicKey;
  let player1Pda: PublicKey;
  let player1PdaAta: PublicKey;
  let player1RpsBasicPda: PublicKey;
  let player2Pda: PublicKey;
  let player2PdaAta: PublicKey;
  let player2RpsBasicPda: PublicKey;
  let gameClientAta: PublicKey;
  let platformAta: PublicKey;

  before(async () => {
    await anchor
      .getProvider()
      .connection.requestAirdrop(admin.publicKey, LAMPORTS_PER_SOL * 1000);
    await anchor
      .getProvider()
      .connection.requestAirdrop(gameClient.publicKey, LAMPORTS_PER_SOL * 1000);

    // Create usdc_mint
    usdcMint = await createMint(
      anchor.getProvider().connection,
      admin,
      admin.publicKey,
      admin.publicKey,
      6
    );

    // Init program
    await program.methods
      .init({
        clientFeeBps: 50, // 0.5%
        platformFeeBps: 50, // 0.5%
        platformKey: platformAcc.publicKey,
      })
      .accounts({
        admin: admin.publicKey,
        usdcMint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();

    managerPda = PublicKey.findProgramAddressSync(
      [Buffer.from("manager")],
      program.programId
    )[0];

    vault = (
      await getOrCreateAssociatedTokenAccount(
        anchor.getProvider().connection,
        admin,
        usdcMint,
        managerPda,
        true
      )
    ).address;

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

    player1PdaAta = (
      await getOrCreateAssociatedTokenAccount(
        anchor.getProvider().connection,
        admin,
        usdcMint,
        player1Pda,
        true
      )
    ).address;

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

    player2PdaAta = (
      await getOrCreateAssociatedTokenAccount(
        anchor.getProvider().connection,
        admin,
        usdcMint,
        player2Pda,
        true
      )
    ).address;

    // Fund players to bet
    await mintTo(
      anchor.getProvider().connection,
      admin,
      usdcMint,
      player1PdaAta,
      admin,
      INIT_PLAYER_BAL.toNumber()
    );

    await mintTo(
      anchor.getProvider().connection,
      admin,
      usdcMint,
      player2PdaAta,
      admin,
      INIT_PLAYER_BAL.toNumber()
    );

    // create platformAcc and gameClient signer ATAs
    platformAta = (
      await getOrCreateAssociatedTokenAccount(
        anchor.getProvider().connection,
        admin,
        usdcMint,
        platformAcc.publicKey
      )
    ).address;

    gameClientAta = (
      await getOrCreateAssociatedTokenAccount(
        anchor.getProvider().connection,
        admin,
        usdcMint,
        gameClient.publicKey
      )
    ).address;
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

    player1RpsBasicPda = PublicKey.findProgramAddressSync(
      [Buffer.from("rps_basic_player"), Buffer.from(PLAYER1_USERNAME)],
      program.programId
    )[0];

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

    player2RpsBasicPda = PublicKey.findProgramAddressSync(
      [Buffer.from("rps_basic_player"), Buffer.from(PLAYER2_USERNAME)],
      program.programId
    )[0];

    // Player1 starts a new game
    // Generate choice hash
    let choice_hash = createHash("sha256")
      .update(Buffer.from(TEST_SECRET))
      .update(gameClientPda.toString())
      .update(gameId.toString())
      .update(player1Choice.toString())
      .digest();

    await program.methods
      .initRpsBasic({
        id: gameId,
        amount: MIN_AMOUNT,
        choiceHash: Array.from(choice_hash),
      })
      .accounts({
        signer: gameClient.publicKey,
        player1: player1Pda,
        gameClient: gameClientPda,
        usdcMint,
        manager: managerPda,
        vault,
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
        usdcMint,
        manager: managerPda,
        vault,
      })
      .signers([gameClient])
      .rpc();

    // confirm player2 joined the game
    let gameData = await program.account.rpsBasicGame.fetch(gamePda);

    assert(gameData.player2, "Player2 did not join the game");

    // Confirm the vault holds right amount
    let vaultBalance = (
      await anchor.getProvider().connection.getTokenAccountBalance(vault)
    ).value.amount;

    assert(
      parseInt(vaultBalance) == MIN_AMOUNT.mul(new BN(2)).toNumber(),
      "Vault balance is wrong"
    );

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
        usdcMint,
        manager: managerPda,
        vault,
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

    // Vault balance should be zero because we took everything out
    let newVaultBalance = (
      await anchor.getProvider().connection.getTokenAccountBalance(vault)
    ).value.amount;

    assert(
      parseInt(newVaultBalance) == 0,
      "Vault balance is not 0 after complete game"
    );

    // Player1 is the winner, so he should have more then the initial balance
    let player1AtaBalanace = (
      await anchor
        .getProvider()
        .connection.getTokenAccountBalance(player1PdaAta)
    ).value.amount;

    assert(
      parseInt(player1AtaBalanace) > INIT_PLAYER_BAL.toNumber(),
      "Player1 balance is incorrect"
    );

    // Player2 lost, so he should have less then initial balance
    let player2AtaBalanace = (
      await anchor
        .getProvider()
        .connection.getTokenAccountBalance(player2PdaAta)
    ).value.amount;

    assert(
      parseInt(player2AtaBalanace) < INIT_PLAYER_BAL.toNumber(),
      "Player1 balance is incorrect"
    );

    // Confirm platform and client got their fee
    let fee = MIN_AMOUNT.mul(new BN(2)).mul(new BN(50)).div(new BN(10000));

    let clientAtaBalanace = (
      await anchor
        .getProvider()
        .connection.getTokenAccountBalance(gameClientAta)
    ).value.amount;

    let platformAtaBalanace = (
      await anchor.getProvider().connection.getTokenAccountBalance(platformAta)
    ).value.amount;

    // confirm platform and client ata holds the exact fee amount (only 1 game was paid)
    assert(
      parseInt(clientAtaBalanace) === fee.toNumber(),
      "Client doesn't hold fee amount"
    );
    assert(
      parseInt(platformAtaBalanace) === fee.toNumber(),
      "Platform doesn't hold fee amount"
    );
  });

  it("Cancel game", async () => {
    let gameId = new BN(1);
    let player1Choice = 1;

    let player1PdaAtaBalanaceInit = parseInt(
      (
        await anchor
          .getProvider()
          .connection.getTokenAccountBalance(player1PdaAta)
      ).value.amount
    );

    let gameClientAtaBalanaceInit = parseInt(
      (
        await anchor
          .getProvider()
          .connection.getTokenAccountBalance(gameClientAta)
      ).value.amount
    );

    let choice_hash = createHash("sha256")
      .update(Buffer.from(TEST_SECRET))
      .update(gameClientPda.toString())
      .update(gameId.toString())
      .update(player1Choice.toString())
      .digest();

    // Start new game to cancel later
    await program.methods
      .initRpsBasic({
        id: gameId,
        amount: MIN_AMOUNT,
        choiceHash: Array.from(choice_hash),
      })
      .accounts({
        signer: gameClient.publicKey,
        player1: player1Pda,
        gameClient: gameClientPda,
        usdcMint,
        manager: managerPda,
        vault,
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

    // Cancle game
    await program.methods
      .cancelRpsBasic()
      .accounts({
        signer: gameClient.publicKey,
        rpsBasicGame: gamePda,
        player1: player1Pda,
        player1RpsBasic: player1RpsBasicPda,
        gameClient: gameClientPda,
        usdcMint,
        manager: managerPda,
        vault,
      })
      .signers([gameClient])
      .rpc();

    let fee = MIN_AMOUNT.mul(new BN(50)).div(new BN(10000));

    let player1PdaAtaBalanace = (
      await anchor
        .getProvider()
        .connection.getTokenAccountBalance(player1PdaAta)
    ).value.amount;

    // Confirm we are back to initial player balance minus the fee
    assert(
      parseInt(player1PdaAtaBalanace) ===
        player1PdaAtaBalanaceInit - fee.toNumber(),
      "Player1 doens't have correct funds"
    );

    let gameClientAtaBalanace = (
      await anchor
        .getProvider()
        .connection.getTokenAccountBalance(gameClientAta)
    ).value.amount;

    console.log(
      gameClientAtaBalanace,
      gameClientAtaBalanaceInit + fee.toNumber()
    );
    assert(
      parseInt(gameClientAtaBalanace) ===
        gameClientAtaBalanaceInit + fee.toNumber(),
      "Client doesn't hold fee amount"
    );
  });
});
