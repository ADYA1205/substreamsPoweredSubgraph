// Code generated by protoc-gen-as. DO NOT EDIT.
// Versions:
//   protoc-gen-as v1.3.3

import { Writer, Reader } from "as-proto/assembly";
import { Timestamp } from "../../google/protobuf/Timestamp";

export class Pimark_SwapProposed {
  static encode(message: Pimark_SwapProposed, writer: Writer): void {
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
    writer.bytes(message.to);

    writer.uint32(50);
    writer.string(message.swapId);

    writer.uint32(58);
    writer.string(message.outTokenId);

    writer.uint32(66);
    writer.bytes(message.outTokenIdAddress);

    writer.uint32(74);
    writer.string(message.inTokenId);

    writer.uint32(82);
    writer.bytes(message.inTokenIdAddress);
  }

  static decode(reader: Reader, length: i32): Pimark_SwapProposed {
    const end: usize = length < 0 ? reader.end : reader.ptr + length;
    const message = new Pimark_SwapProposed();

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
          message.to = reader.bytes();
          break;

        case 6:
          message.swapId = reader.string();
          break;

        case 7:
          message.outTokenId = reader.string();
          break;

        case 8:
          message.outTokenIdAddress = reader.bytes();
          break;

        case 9:
          message.inTokenId = reader.string();
          break;

        case 10:
          message.inTokenIdAddress = reader.bytes();
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
  to: Uint8Array;
  swapId: string;
  outTokenId: string;
  outTokenIdAddress: Uint8Array;
  inTokenId: string;
  inTokenIdAddress: Uint8Array;

  constructor(
    evtTxHash: string = "",
    evtIndex: u32 = 0,
    evtBlockTime: Timestamp | null = null,
    evtBlockNumber: u64 = 0,
    to: Uint8Array = new Uint8Array(0),
    swapId: string = "",
    outTokenId: string = "",
    outTokenIdAddress: Uint8Array = new Uint8Array(0),
    inTokenId: string = "",
    inTokenIdAddress: Uint8Array = new Uint8Array(0)
  ) {
    this.evtTxHash = evtTxHash;
    this.evtIndex = evtIndex;
    this.evtBlockTime = evtBlockTime;
    this.evtBlockNumber = evtBlockNumber;
    this.to = to;
    this.swapId = swapId;
    this.outTokenId = outTokenId;
    this.outTokenIdAddress = outTokenIdAddress;
    this.inTokenId = inTokenId;
    this.inTokenIdAddress = inTokenIdAddress;
  }
}
