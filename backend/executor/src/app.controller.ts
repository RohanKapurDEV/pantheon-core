import { Body, Controller, Get, Post } from '@nestjs/common';
import { AppService } from './app.service';
import { SolanaService } from './solana/solana.service';
import { DatabaseService } from './database/database.service';
import { DcaMetadata, PaymentSchedule } from '@prisma/client';
import { selectCluster } from './utils';

@Controller()
export class AppController {
  constructor(
    private readonly appService: AppService,
    private readonly solanaService: SolanaService,
    private readonly databaseService: DatabaseService,
  ) {}

  @Post('/swap')
  async performDcaSwapAndReturn(@Body() req: PerformDcaSwapAndReturnRequest) {
    const paymentScheduleId = req.paymentScheduleId;

    let paymentSchedule: PaymentSchedule;
    let dcaMetadata: DcaMetadata;

    try {
      const tryFetchPaymentSchedule =
        await this.databaseService.readPaymentSchedule({
          payment_schedule_id: paymentScheduleId,
        });

      if (!tryFetchPaymentSchedule) {
        throw new Error('Payment schedule not found');
      }

      paymentSchedule = tryFetchPaymentSchedule;
    } catch (error) {
      throw new Error('2 Payment schedule not found. Error: ' + error);
    }

    try {
      const tryFetchDcaMetadata = await this.databaseService.readDcaMetadata({
        dca_metadata_id: paymentSchedule.dca_metadata_id,
      });

      if (!tryFetchDcaMetadata) {
        throw new Error('DCA metadata not found');
      }

      dcaMetadata = tryFetchDcaMetadata;
    } catch (error) {
      throw new Error('2 DCA metadata not found. Error: ' + error);
    }

    const network = selectCluster(paymentSchedule.network);

    let withdrawalTxId: string;
    let swapTxId: string;
    let returnTxId: string;
    let inputAmount: number;
    let outputAmount: number;

    try {
      const tryWithdrawal =
        await this.solanaService.withdrawTokenFromPantheonVault({
          network: network,
          vaultFromTokenMint: dcaMetadata.from_token_mint,
          vaultToTokenMint: dcaMetadata.to_token_mint,
          dcaMetadataAddress: paymentSchedule.dca_metadata_address,
        });

      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      withdrawalTxId = tryWithdrawal;
    } catch (error) {
      throw new Error('Withdrawal failed. Error: ' + error);
    }

    try {
      const trySwap = await this.solanaService.performSwap({
        network: network,
        inputAmount: Number(dcaMetadata.amount_per_interval), // This is pretty unsafe to do tbh, but it'll do for now
        vaultFromTokenMint: dcaMetadata.from_token_mint,
        vaultToTokenMint: dcaMetadata.to_token_mint,
        slippage: parseFloat(dcaMetadata.slippage),
      });

      swapTxId = trySwap[0];
      inputAmount = trySwap[1];
      outputAmount = trySwap[2];
    } catch (error) {
      throw new Error('Swap failed. Error: ' + error);
    }

    try {
      const tryReturn = await this.solanaService.depositBackToPantheonVault({
        network: network,
        dcaMetadataAddress: paymentSchedule.dca_metadata_address,
        outputAmount: outputAmount,
        vaultToTokenMint: dcaMetadata.to_token_mint,
      });

      returnTxId = tryReturn;
    } catch (error) {
      throw new Error('Return failed. Error: ' + error);
    }

    try {
      await this.databaseService.deletePaymentSchedule({
        payment_schedule_id: paymentScheduleId,
      });

      await this.databaseService.createProcessedSchedule({
        dca_metadata_address: paymentSchedule.dca_metadata_address,
        dca_metadata_id: paymentSchedule.dca_metadata_id,
        inputAmount: inputAmount,
        outputAmount: outputAmount,
        network: network,
        tx_sig: swapTxId,
      });
    } catch (error) {
      throw new Error('Processing failed. Error: ' + error);
    }

    return true;
  }

  // Fetch the pubkey of the current CrankAuthority configured into the deploy environment
  @Get('/crankAuthority')
  async postRandomNumber(): Promise<string> {
    return await this.solanaService.currentCrankAuthority();
  }
}

export interface PerformDcaSwapAndReturnRequest {
  paymentScheduleId: bigint;
}
