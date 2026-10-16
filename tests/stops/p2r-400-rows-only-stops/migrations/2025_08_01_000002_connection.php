<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Support\Facades\DB;

// The connection of a DB::table chain runs any statement, so this file changes columns.
return new class extends Migration
{
    public function up(): void
    {
        DB::table('pins')->getConnection()->statement('ALTER TABLE pins DROP COLUMN name');
    }
};
