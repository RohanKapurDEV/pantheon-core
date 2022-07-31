import { Module } from '@nestjs/common';
import { SolanaService } from './solana.service';

@Module({
  imports: [],
  providers: [SolanaService],
  exports: [SolanaService],
})
export class SolanaModule {}
