// Code generated by protoc-gen-as. DO NOT EDIT.
// Versions:
//   protoc-gen-as v1.3.3

import { Writer, Reader } from "as-proto/assembly";
import { Timestamp } from "../../google/protobuf/Timestamp";

export class Pimeth_ValidatorFundsWithdrawn {
  static encode(message: Pimeth_ValidatorFundsWithdrawn, writer: Writer): void {
    writer.uint32(10);
    writer.string(message.evtTxHash);

    writer.uint32(16);
    writer.uint32(message.evtIndex);

    const evtBlockTime = message.evtBlockTime;
    if (evtBlockTime !== null) {
      writer.uint32(26);
      writer.fork();
      Timestamp.encode(evtBlockTime, writer);
      writer.ldelim();
    }

    writer.uint32(32);
    writer.uint64(message.evtBlockNumber);

    writer.uint32(42);
    writer.bytes(message.collectionAddress);

    writer.uint32(50);
    writer.bytes(message.withdrawer);

    writer.uint32(58);
    writer.string(message.tokenId);

    writer.uint32(66);
    writer.bytes(message.erc20Contract);

    writer.uint32(74);
    writer.string(message.amount);
  }

  static decode(reader: Reader, length: i32): Pimeth_ValidatorFundsWithdrawn {
    const end: usize = length < 0 ? reader.end : reader.ptr + length;
    const message = new Pimeth_ValidatorFundsWithdrawn();

    while (reader.ptr < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.evtTxHash = reader.string();
          break;

        case 2:
          message.evtIndex = reader.uint32();
          break;

        case 3:
          message.evtBlockTime = Timestamp.decode(reader, reader.uint32());
          break;

        case 4:
          message.evtBlockNumber = reader.uint64();
          break;

        case 5:
          message.collectionAddress = reader.bytes();
          break;

        case 6:
          message.withdrawer = reader.bytes();
          break;

        case 7:
          message.tokenId = reader.string();
          break;

        case 8:
          message.erc20Contract = reader.bytes();
          break;

        case 9:
          message.amount = reader.string();
          break;

        default:
          reader.skipType(tag & 7);
          break;
      }
    }

    return message;
  }

  evtTxHash: string;
  evtIndex: u32;
  evtBlockTime: Timestamp | null;
  evtBlockNumber: u64;
  collectionAddress: Uint8Array;
  withdrawer: Uint8Array;
  tokenId: string;
  erc20Contract: Uint8Array;
  amount: string;

  constructor(
    evtTxHash: string = "",
    evtIndex: u32 = 0,
    evtBlockTime: Timestamp | null = null,
    evtBlockNumber: u64 = 0,
    collectionAddress: Uint8Array = new Uint8Array(0),
    withdrawer: Uint8Array = new Uint8Array(0),
    tokenId: string = "",
    erc20Contract: Uint8Array = new Uint8Array(0),
    amount: string = ""
  ) {
    this.evtTxHash = evtTxHash;
    this.evtIndex = evtIndex;
    this.evtBlockTime = evtBlockTime;
    this.evtBlockNumber = evtBlockNumber;
    this.collectionAddress = collectionAddress;
    this.withdrawer = withdrawer;
    this.tokenId = tokenId;
    this.erc20Contract = erc20Contract;
    this.amount = amount;
  }
}
