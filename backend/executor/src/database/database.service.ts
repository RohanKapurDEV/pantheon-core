import { Injectable } from '@nestjs/common';
import { PrismaService } from './prisma.service';
import {
  DcaMetadata,
  PaymentSchedule,
  Prisma,
  ProcessedSchedule,
} from '@prisma/client';

@Injectable()
export class DatabaseService {
  constructor(private prisma: PrismaService) {}

  async createProcessedSchedule(
    account: Prisma.ProcessedScheduleCreateInput,
  ): Promise<ProcessedSchedule | null> {
    return this.prisma.processedSchedule.create({
      data: account,
    });
  }

  async readDcaMetadata(
    accountWhereUniqueInput: Prisma.DcaMetadataWhereUniqueInput,
  ): Promise<DcaMetadata | null> {
    return this.prisma.dcaMetadata.findUnique({
      where: accountWhereUniqueInput,
    });
  }

  async readPaymentSchedule(
    accountWhereUniqueInput: Prisma.PaymentScheduleWhereUniqueInput,
  ): Promise<PaymentSchedule | null> {
    return this.prisma.paymentSchedule.findUnique({
      where: accountWhereUniqueInput,
    });
  }

  async deletePaymentSchedule(
    accountWhereUniqueInput: Prisma.PaymentScheduleWhereUniqueInput,
  ): Promise<PaymentSchedule | null> {
    return this.prisma.paymentSchedule.delete({
      where: accountWhereUniqueInput,
    });
  }
}
