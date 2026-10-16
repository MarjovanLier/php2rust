<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Support\Facades\DB;

// The database holds this row, which the store, empty when it starts, does not.
return new class extends Migration
{
    public function up(): void
    {
        DB::table('seededs')->insert(['name' => 'first']);
    }
};
