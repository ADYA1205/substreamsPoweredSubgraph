// Code generated by protoc-gen-as. DO NOT EDIT.
// Versions:
//   protoc-gen-as v1.3.3

import { Writer, Reader } from "as-proto/assembly";
import { Timestamp } from "../../google/protobuf/Timestamp";

export class Pimark_MakeSwapRequestCall {
  static encode(message: Pimark_MakeSwapRequestCall, writer: Writer): void {
    writer.uint32(10);
    writer.string(message.callTxHash);

    const callBlockTime = message.callBlockTime;
    if (callBlockTime !== null) {
      writer.uint32(18);
      writer.fork();
      Timestamp.encode(callBlockTime, writer);
      writer.ldelim();
    }

    writer.uint32(24);
    writer.uint64(message.callBlockNumber);

    writer.uint32(32);
    writer.uint64(message.callOrdinal);

    writer.uint32(40);
    writer.bool(message.callSuccess);

    writer.uint32(50);
    writer.bytes(message.contractAddress1);

    writer.uint32(58);
    writer.bytes(message.contractAddress2);

    writer.uint32(66);
    writer.string(message.token1);

    writer.uint32(74);
    writer.string(message.token2);

    writer.uint32(82);
    writer.string(message.outputParam0);
  }

  static decode(reader: Reader, length: i32): Pimark_MakeSwapRequestCall {
    const end: usize = length < 0 ? reader.end : reader.ptr + length;
    const message = new Pimark_MakeSwapRequestCall();

    while (reader.ptr < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.callTxHash = reader.string();
          break;

        case 2:
          message.callBlockTime = Timestamp.decode(reader, reader.uint32());
          break;

        case 3:
          message.callBlockNumber = reader.uint64();
          break;

        case 4:
          message.callOrdinal = reader.uint64();
          break;

        case 5:
          message.callSuccess = reader.bool();
          break;

        case 6:
          message.contractAddress1 = reader.bytes();
          break;

        case 7:
          message.contractAddress2 = reader.bytes();
          break;

        case 8:
          message.token1 = reader.string();
          break;

        case 9:
          message.token2 = reader.string();
          break;

        case 10:
          message.outputParam0 = reader.string();
          break;

        default:
          reader.skipType(tag & 7);
          break;
      }
    }

    return message;
  }

  callTxHash: string;
  callBlockTime: Timestamp | null;
  callBlockNumber: u64;
  callOrdinal: u64;
  callSuccess: bool;
  contractAddress1: Uint8Array;
  contractAddress2: Uint8Array;
  token1: string;
  token2: string;
  outputParam0: string;

  constructor(
    callTxHash: string = "",
    callBlockTime: Timestamp | null = null,
    callBlockNumber: u64 = 0,
    callOrdinal: u64 = 0,
    callSuccess: bool = false,
    contractAddress1: Uint8Array = new Uint8Array(0),
    contractAddress2: Uint8Array = new Uint8Array(0),
    token1: string = "",
    token2: string = "",
    outputParam0: string = ""
  ) {
    this.callTxHash = callTxHash;
    this.callBlockTime = callBlockTime;
    this.callBlockNumber = callBlockNumber;
    this.callOrdinal = callOrdinal;
    this.callSuccess = callSuccess;
    this.contractAddress1 = contractAddress1;
    this.contractAddress2 = contractAddress2;
    this.token1 = token1;
    this.token2 = token2;
    this.outputParam0 = outputParam0;
  }
}