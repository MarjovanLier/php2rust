<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('location_codes', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('code');
            $table->string('secret');
        });
        Schema::create('events', function (Blueprint $table) {
            $table->id();
            $table->string('label');
        });
    }
};
