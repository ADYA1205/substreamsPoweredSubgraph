// Code generated by protoc-gen-as. DO NOT EDIT.
// Versions:
//   protoc-gen-as v1.3.3

import { Writer, Reader } from "as-proto/assembly";
import { Timestamp } from "../../google/protobuf/Timestamp";

export class Pimark_SaleCreated {
  static encode(message: Pimark_SaleCreated, writer: Writer): void {
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
    writer.string(message.tokenId);

    writer.uint32(50);
    writer.bytes(message.tokenContract);

    writer.uint32(58);
    writer.string(message.saleId);

    writer.uint32(66);
    writer.string(message.bidTimeDuration);

    writer.uint32(74);
    writer.string(message.price);
  }

  static decode(reader: Reader, length: i32): Pimark_SaleCreated {
    const end: usize = length < 0 ? reader.end : reader.ptr + length;
    const message = new Pimark_SaleCreated();

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
          message.tokenId = reader.string();
          break;

        case 6:
          message.tokenContract = reader.bytes();
          break;

        case 7:
          message.saleId = reader.string();
          break;

        case 8:
          message.bidTimeDuration = reader.string();
          break;

        case 9:
          message.price = reader.string();
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
  tokenId: string;
  tokenContract: Uint8Array;
  saleId: string;
  bidTimeDuration: string;
  price: string;

  constructor(
    evtTxHash: string = "",
    evtIndex: u32 = 0,
    evtBlockTime: Timestamp | null = null,
    evtBlockNumber: u64 = 0,
    tokenId: string = "",
    tokenContract: Uint8Array = new Uint8Array(0),
    saleId: string = "",
    bidTimeDuration: string = "",
    price: string = ""
  ) {
    this.evtTxHash = evtTxHash;
    this.evtIndex = evtIndex;
    this.evtBlockTime = evtBlockTime;
    this.evtBlockNumber = evtBlockNumber;
    this.tokenId = tokenId;
    this.tokenContract = tokenContract;
    this.saleId = saleId;
    this.bidTimeDuration = bidTimeDuration;
    this.price = price;
  }
}