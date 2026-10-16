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
            $table->string('code');
            $table->integer('exists')->default(0);
            $table->boolean('active');
            $table->decimal('price', 8, 2);
            $table->string('status');
            $table->timestamps();
        });
        Schema::create('categories', function (Blueprint $table) {
            $table->id();
            $table->string('label')->nullable();
        });
        Schema::create('boxes', function (Blueprint $table) {
            $table->id();
            $table->unsignedTinyInteger('size');
        });
        Schema::create('legacy_items', function (Blueprint $table) {
            $table->id();
            $table->text('body');
        });
    }
};
