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
        Schema::create('cogs', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        // A callback that calls Schema makes the chain another statement.
        DB::table('cogs')->orderBy('id')->chunk(100, function ($rows) {
            Schema::dropColumns('cogs', ['name']);
        });
        Schema::create('belts', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
    }
};
