<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('accounts', function (Blueprint $table) {
            $table->id();
            $table->string('uuid');
            $table->string('name');
        });
        Schema::create('codes', function (Blueprint $table) {
            $table->id();
            $table->string('code');
        });
        Schema::create('sources', function (Blueprint $table) {
            $table->id();
            $table->string('label');
        });
    }
};
