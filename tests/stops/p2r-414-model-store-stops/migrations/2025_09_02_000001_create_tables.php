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
            $table->string('name');
            $table->string('code')->nullable()->default('x');
            $table->integer('size');
        });
        Schema::create('lockeds', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        Schema::create('partials', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('code');
            $table->integer('size');
        });
        // A column that cannot be null, has no default and has no field.
        Schema::create('dateds', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->date('day');
        });
        // A key that the database does not number.
        Schema::create('keyeds', function (Blueprint $table) {
            $table->integer('id');
            $table->string('name');
        });
        Schema::create('stampeds', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        Schema::create('opens', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('_hash')->nullable();
        });
        Schema::create('amounts', function (Blueprint $table) {
            $table->id();
            $table->integer('amount');
        });
        Schema::create('pairs', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        Schema::create('mappeds', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        Schema::create('measures', function (Blueprint $table) {
            $table->id();
            $table->double('size');
        });
        Schema::create('flaggeds', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->boolean('active')->useCurrent();
        });
        Schema::create('seededs', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        // A SoftDeletes model needs deleted_at, which Laravel's queries read.
        Schema::create('softs', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
    }
};
