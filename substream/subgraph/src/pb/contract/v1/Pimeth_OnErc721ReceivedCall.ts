// Code generated by protoc-gen-as. DO NOT EDIT.
// Versions:
//   protoc-gen-as v1.3.3

import { Writer, Reader } from "as-proto/assembly";
import { Timestamp } from "../../google/protobuf/Timestamp";

export class Pimeth_OnErc721ReceivedCall {
  static encode(message: Pimeth_OnErc721ReceivedCall, writer: Writer): void {
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
    writer.bytes(message.param0);

    writer.uint32(58);
    writer.bytes(message.param1);

    writer.uint32(66);
    writer.string(message.param2);

    writer.uint32(74);
    writer.bytes(message.param3);

    writer.uint32(82);
    writer.bytes(message.outputParam0);
  }

  static decode(reader: Reader, length: i32): Pimeth_OnErc721ReceivedCall {
    const end: usize = length < 0 ? reader.end : reader.ptr + length;
    const message = new Pimeth_OnErc721ReceivedCall();

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
          message.param0 = reader.bytes();
          break;

        case 7:
          message.param1 = reader.bytes();
          break;

        case 8:
          message.param2 = reader.string();
          break;

        case 9:
          message.param3 = reader.bytes();
          break;

        case 10:
          message.outputParam0 = reader.bytes();
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
  param0: Uint8Array;
  param1: Uint8Array;
  param2: string;
  param3: Uint8Array;
  outputParam0: Uint8Array;

  constructor(
    callTxHash: string = "",
    callBlockTime: Timestamp | null = null,
    callBlockNumber: u64 = 0,
    callOrdinal: u64 = 0,
    callSuccess: bool = false,
    param0: Uint8Array = new Uint8Array(0),
    param1: Uint8Array = new Uint8Array(0),
    param2: string = "",
    param3: Uint8Array = new Uint8Array(0),
    outputParam0: Uint8Array = new Uint8Array(0)
  ) {
    this.callTxHash = callTxHash;
    this.callBlockTime = callBlockTime;
    this.callBlockNumber = callBlockNumber;
    this.callOrdinal = callOrdinal;
    this.callSuccess = callSuccess;
    this.param0 = param0;
    this.param1 = param1;
    this.param2 = param2;
    this.param3 = param3;
    this.outputParam0 = outputParam0;
  }
}
