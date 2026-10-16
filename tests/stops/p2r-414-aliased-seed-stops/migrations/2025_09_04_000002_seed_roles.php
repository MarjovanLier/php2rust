<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Support\Facades\DB;

// SQLite reads `roles as r` as the table roles.
return new class extends Migration
{
    public function up(): void
    {
        DB::table('roles as r')->insert(['name' => 'admin']);
    }
};
