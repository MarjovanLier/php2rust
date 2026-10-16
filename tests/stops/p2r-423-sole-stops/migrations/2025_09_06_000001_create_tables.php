<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('tags', function (Blueprint $table) {
            $table->id();
            $table->string('label');
        });
        // A key that the database does not number.
        Schema::create('keyeds', function (Blueprint $table) {
            $table->integer('id');
            $table->string('label');
        });
    }
};
