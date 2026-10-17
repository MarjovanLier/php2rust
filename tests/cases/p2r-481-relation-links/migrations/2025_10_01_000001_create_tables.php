<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('roles', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        Schema::create('permissions', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        Schema::create('addresses', function (Blueprint $table) {
            $table->id();
            $table->string('street');
        });
    }
};
