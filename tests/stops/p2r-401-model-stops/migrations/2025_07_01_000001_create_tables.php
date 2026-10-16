<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('items', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        // Two columns whose Rust names are the same.
        Schema::create('pairs', function (Blueprint $table) {
            $table->id();
            $table->string('userId');
            $table->string('user_id');
        });
    }
};
