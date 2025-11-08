import * as anchor from "@coral-xyz/anchor";

describe("tweet_program", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.TweetProgram;

  const user = provider.wallet.publicKey;

  const [tweetPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("tweet"), user.toBuffer()],
    program.programId
  );

  it("creates a tweet", async () => {
    await program.methods
      .createTweet("Hello Solana world")
      .accounts({
        tweet: tweetPda,
        user: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const acc = await program.account.tweet.fetch(tweetPda);

    console.log("author:", acc.author.toBase58());
    console.log("timestamp:", acc.timestamp.toString());
    console.log("text:", acc.text);
  });
});
