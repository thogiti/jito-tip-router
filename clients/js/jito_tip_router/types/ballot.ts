/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  fixDecoderSize,
  fixEncoderSize,
  getBytesDecoder,
  getBytesEncoder,
  getStructDecoder,
  getStructEncoder,
  type Codec,
  type Decoder,
  type Encoder,
  type ReadonlyUint8Array,
} from '@solana/web3.js';

export type Ballot = {
  merkleRoot: ReadonlyUint8Array;
  reserved: ReadonlyUint8Array;
};

export type BallotArgs = Ballot;

export function getBallotEncoder(): Encoder<BallotArgs> {
  return getStructEncoder([
    ['merkleRoot', fixEncoderSize(getBytesEncoder(), 32)],
    ['reserved', fixEncoderSize(getBytesEncoder(), 64)],
  ]);
}

export function getBallotDecoder(): Decoder<Ballot> {
  return getStructDecoder([
    ['merkleRoot', fixDecoderSize(getBytesDecoder(), 32)],
    ['reserved', fixDecoderSize(getBytesDecoder(), 64)],
  ]);
}

export function getBallotCodec(): Codec<BallotArgs, Ballot> {
  return combineCodec(getBallotEncoder(), getBallotDecoder());
}
