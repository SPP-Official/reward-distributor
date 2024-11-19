import {
  PublicKey,
  AccountMeta,
  TransactionInstruction,
} from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";
import BN from "bn.js";

export const REWARD_PROGRAM_ID = new PublicKey(
  "PLAYcZHpkkcLiWY2Csw6bcUbbHh85T3tCqnwsA4qBwh"
);
export const SYSTEM_PROGRAM_ID = new PublicKey(
  "11111111111111111111111111111111"
);

export const TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
);

export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey(
  "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
);

export const RENT_PROGRAM_ID = new PublicKey(
  "SysvarRent111111111111111111111111111111111"
);

export const rewardArgsLayout = borsh.struct([borsh.u64("amount")]);

export const INIT_IDENTIFIER = [0];

export const REWARD_IDENTIFIER = [1];

export const getAtaKey = (owner: PublicKey, mint: PublicKey): PublicKey => {
  return PublicKey.findProgramAddressSync(
    [owner.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID
  )[0];
};

export const initializeInstruction = (
  authority: PublicKey,
  mint: PublicKey
): TransactionInstruction => {
  const [vaultPda, _] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault")],
    REWARD_PROGRAM_ID
  );

  const mintAta = getAtaKey(vaultPda, mint);

  const keys: Array<AccountMeta> = [
    { pubkey: authority, isSigner: true, isWritable: true },
    { pubkey: mint, isSigner: false, isWritable: false },
    { pubkey: vaultPda, isSigner: false, isWritable: true },
    { pubkey: mintAta, isSigner: false, isWritable: true },
    { pubkey: SYSTEM_PROGRAM_ID, isSigner: false, isWritable: false },
    { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    { pubkey: RENT_PROGRAM_ID, isSigner: false, isWritable: false },
    { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
  ];

  const identifier = Buffer.from(INIT_IDENTIFIER);
  const data = Buffer.concat([identifier], 1);

  const instruction = new TransactionInstruction({
    keys,
    programId: REWARD_PROGRAM_ID,
    data,
  });

  return instruction;
};

export const rewardInstruction = (
  authority: PublicKey,
  mint: PublicKey,
  recipient: PublicKey,
  amount: BN
): TransactionInstruction => {
  const [vaultPda, _] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault")],
    REWARD_PROGRAM_ID
  );

  const mintAta = getAtaKey(vaultPda, mint);
  const destinationAta = getAtaKey(recipient, mint);

  const keys: Array<AccountMeta> = [
    { pubkey: authority, isSigner: true, isWritable: true },
    { pubkey: mint, isSigner: false, isWritable: false },
    { pubkey: vaultPda, isSigner: false, isWritable: true },
    { pubkey: mintAta, isSigner: false, isWritable: true },
    { pubkey: destinationAta, isSigner: false, isWritable: true },
    { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
  ];

  const identifier = Buffer.from(REWARD_IDENTIFIER);
  const buffer = Buffer.alloc(1000);
  const len = rewardArgsLayout.encode({ amount }, buffer);
  const data = Buffer.concat([identifier, buffer], 1 + len);

  const instruction = new TransactionInstruction({
    keys,
    programId: REWARD_PROGRAM_ID,
    data,
  });

  return instruction;
};
