<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('authors', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('title')->nullable();
        });
        Schema::create('posts', function (Blueprint $table) {
            $table->id();
            $table->string('title');
        });
        Schema::create('accessors', function (Blueprint $table) {
            $table->id();
            $table->string('title');
        });
    }
};
