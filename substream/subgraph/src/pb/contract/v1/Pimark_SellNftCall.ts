// Code generated by protoc-gen-as. DO NOT EDIT.
// Versions:
//   protoc-gen-as v1.3.3

import { Writer, Reader } from "as-proto/assembly";
import { Timestamp } from "../../google/protobuf/Timestamp";

export class Pimark_SellNftCall {
  static encode(message: Pimark_SellNftCall, writer: Writer): void {
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
    writer.bytes(message.uContractAddress);

    writer.uint32(58);
    writer.string(message.uTokenId);

    writer.uint32(66);
    writer.string(message.uPrice);

    writer.uint32(74);
    writer.bytes(message.uCurrency);
  }

  static decode(reader: Reader, length: i32): Pimark_SellNftCall {
    const end: usize = length < 0 ? reader.end : reader.ptr + length;
    const message = new Pimark_SellNftCall();

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
          message.uContractAddress = reader.bytes();
          break;

        case 7:
          message.uTokenId = reader.string();
          break;

        case 8:
          message.uPrice = reader.string();
          break;

        case 9:
          message.uCurrency = reader.bytes();
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
  uContractAddress: Uint8Array;
  uTokenId: string;
  uPrice: string;
  uCurrency: Uint8Array;

  constructor(
    callTxHash: string = "",
    callBlockTime: Timestamp | null = null,
    callBlockNumber: u64 = 0,
    callOrdinal: u64 = 0,
    callSuccess: bool = false,
    uContractAddress: Uint8Array = new Uint8Array(0),
    uTokenId: string = "",
    uPrice: string = "",
    uCurrency: Uint8Array = new Uint8Array(0)
  ) {
    this.callTxHash = callTxHash;
    this.callBlockTime = callBlockTime;
    this.callBlockNumber = callBlockNumber;
    this.callOrdinal = callOrdinal;
    this.callSuccess = callSuccess;
    this.uContractAddress = uContractAddress;
    this.uTokenId = uTokenId;
    this.uPrice = uPrice;
    this.uCurrency = uCurrency;
  }
}