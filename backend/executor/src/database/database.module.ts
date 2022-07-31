import { Module } from '@nestjs/common';
import { DatabaseService } from './database.service';
import { PrismaService } from './prisma.service';

@Module({
  exports: [PrismaService, DatabaseService],
  providers: [PrismaService, DatabaseService],
})
export class DatabaseModule {}
