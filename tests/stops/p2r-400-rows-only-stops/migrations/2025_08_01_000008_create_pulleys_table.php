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
        Schema::create('pulleys', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        // The processor runs its SQL through the connection.
        DB::table('pulleys')->getProcessor()->processInsertGetId(
            DB::table('pulleys'),
            'ALTER TABLE pulleys DROP COLUMN name',
            [],
        );
        Schema::create('springs', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
    }
};
