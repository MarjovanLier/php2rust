<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('codes', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        Schema::create('owns', function (Blueprint $table) {
            $table->id();
        });
        Schema::create('uppers', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        Schema::create('inheriteds', function (Blueprint $table) {
            $table->id();
        });
        Schema::create('traiteds', function (Blueprint $table) {
            $table->id();
        });
        Schema::create('hiddens', function (Blueprint $table) {
            $table->id();
        });
        Schema::create('peers', function (Blueprint $table) {
            $table->id();
        });
        Schema::create('private_parenteds', function (Blueprint $table) {
            $table->id();
        });
        Schema::create('taggeds', function (Blueprint $table) {
            $table->id();
        });
    }
};
