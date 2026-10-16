<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        // A raw statement may change any table, so each one that exists becomes unknown.
        DB::statement('ALTER TABLE bolts ADD size INT');
        Schema::create('nuts', function (Blueprint $table) {
            $table->id();
        });
        // After that statement, washers may exist without the tool knowing.
        if (! Schema::hasTable('washers')) {
            Schema::create('washers', function (Blueprint $table) {
                $table->id();
            });
        }
    }
};
