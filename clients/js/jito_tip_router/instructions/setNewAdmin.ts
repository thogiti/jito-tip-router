/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
} from '@solana/web3.js';
import { JITO_TIP_ROUTER_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';
import {
  getConfigAdminRoleDecoder,
  getConfigAdminRoleEncoder,
  type ConfigAdminRole,
  type ConfigAdminRoleArgs,
} from '../types';

export const SET_NEW_ADMIN_DISCRIMINATOR = 2;

export function getSetNewAdminDiscriminatorBytes() {
  return getU8Encoder().encode(SET_NEW_ADMIN_DISCRIMINATOR);
}

export type SetNewAdminInstruction<
  TProgram extends string = typeof JITO_TIP_ROUTER_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountNcn extends string | IAccountMeta<string> = string,
  TAccountNcnAdmin extends string | IAccountMeta<string> = string,
  TAccountNewAdmin extends string | IAccountMeta<string> = string,
  TAccountRestakingProgramId extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountConfig extends string
        ? WritableAccount<TAccountConfig>
        : TAccountConfig,
      TAccountNcn extends string ? ReadonlyAccount<TAccountNcn> : TAccountNcn,
      TAccountNcnAdmin extends string
        ? ReadonlySignerAccount<TAccountNcnAdmin> &
            IAccountSignerMeta<TAccountNcnAdmin>
        : TAccountNcnAdmin,
      TAccountNewAdmin extends string
        ? ReadonlyAccount<TAccountNewAdmin>
        : TAccountNewAdmin,
      TAccountRestakingProgramId extends string
        ? ReadonlyAccount<TAccountRestakingProgramId>
        : TAccountRestakingProgramId,
      ...TRemainingAccounts,
    ]
  >;

export type SetNewAdminInstructionData = {
  discriminator: number;
  role: ConfigAdminRole;
};

export type SetNewAdminInstructionDataArgs = { role: ConfigAdminRoleArgs };

export function getSetNewAdminInstructionDataEncoder(): Encoder<SetNewAdminInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['role', getConfigAdminRoleEncoder()],
    ]),
    (value) => ({ ...value, discriminator: SET_NEW_ADMIN_DISCRIMINATOR })
  );
}

export function getSetNewAdminInstructionDataDecoder(): Decoder<SetNewAdminInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['role', getConfigAdminRoleDecoder()],
  ]);
}

export function getSetNewAdminInstructionDataCodec(): Codec<
  SetNewAdminInstructionDataArgs,
  SetNewAdminInstructionData
> {
  return combineCodec(
    getSetNewAdminInstructionDataEncoder(),
    getSetNewAdminInstructionDataDecoder()
  );
}

export type SetNewAdminInput<
  TAccountConfig extends string = string,
  TAccountNcn extends string = string,
  TAccountNcnAdmin extends string = string,
  TAccountNewAdmin extends string = string,
  TAccountRestakingProgramId extends string = string,
> = {
  config: Address<TAccountConfig>;
  ncn: Address<TAccountNcn>;
  ncnAdmin: TransactionSigner<TAccountNcnAdmin>;
  newAdmin: Address<TAccountNewAdmin>;
  restakingProgramId: Address<TAccountRestakingProgramId>;
  role: SetNewAdminInstructionDataArgs['role'];
};

export function getSetNewAdminInstruction<
  TAccountConfig extends string,
  TAccountNcn extends string,
  TAccountNcnAdmin extends string,
  TAccountNewAdmin extends string,
  TAccountRestakingProgramId extends string,
  TProgramAddress extends Address = typeof JITO_TIP_ROUTER_PROGRAM_ADDRESS,
>(
  input: SetNewAdminInput<
    TAccountConfig,
    TAccountNcn,
    TAccountNcnAdmin,
    TAccountNewAdmin,
    TAccountRestakingProgramId
  >,
  config?: { programAddress?: TProgramAddress }
): SetNewAdminInstruction<
  TProgramAddress,
  TAccountConfig,
  TAccountNcn,
  TAccountNcnAdmin,
  TAccountNewAdmin,
  TAccountRestakingProgramId
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? JITO_TIP_ROUTER_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: true },
    ncn: { value: input.ncn ?? null, isWritable: false },
    ncnAdmin: { value: input.ncnAdmin ?? null, isWritable: false },
    newAdmin: { value: input.newAdmin ?? null, isWritable: false },
    restakingProgramId: {
      value: input.restakingProgramId ?? null,
      isWritable: false,
    },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.config),
      getAccountMeta(accounts.ncn),
      getAccountMeta(accounts.ncnAdmin),
      getAccountMeta(accounts.newAdmin),
      getAccountMeta(accounts.restakingProgramId),
    ],
    programAddress,
    data: getSetNewAdminInstructionDataEncoder().encode(
      args as SetNewAdminInstructionDataArgs
    ),
  } as SetNewAdminInstruction<
    TProgramAddress,
    TAccountConfig,
    TAccountNcn,
    TAccountNcnAdmin,
    TAccountNewAdmin,
    TAccountRestakingProgramId
  >;

  return instruction;
}

export type ParsedSetNewAdminInstruction<
  TProgram extends string = typeof JITO_TIP_ROUTER_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    config: TAccountMetas[0];
    ncn: TAccountMetas[1];
    ncnAdmin: TAccountMetas[2];
    newAdmin: TAccountMetas[3];
    restakingProgramId: TAccountMetas[4];
  };
  data: SetNewAdminInstructionData;
};

export function parseSetNewAdminInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedSetNewAdminInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 5) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      config: getNextAccount(),
      ncn: getNextAccount(),
      ncnAdmin: getNextAccount(),
      newAdmin: getNextAccount(),
      restakingProgramId: getNextAccount(),
    },
    data: getSetNewAdminInstructionDataDecoder().decode(instruction.data),
  };
}
