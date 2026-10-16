<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        // A Blueprint method the tool does not read.
        Schema::create('widgets', function (Blueprint $table) {
            $table->id();
            $table->json('data');
        });
        // On a plain column, Laravel keeps nullable() after constrained() on the column.
        Schema::create('gadgets', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('parent_id')->constrained('parents')->nullable();
        });
        Schema::create('sprockets', function (Blueprint $table) {
            $table->id();
        });
        Schema::table('sprockets', function (Blueprint $table) {
            $table->string('missing')->change();
        });
        Schema::create('levers', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('name');
        });
        Schema::create('pulleys', function (Blueprint $table) {
            $table->id();
        });
        Schema::create('pulleys', function (Blueprint $table) {
            $table->id();
        });
        Schema::table('gears', function (Blueprint $table) {
            $table->string('name');
        });
        Schema::create('bolts', function (Blueprint $table) {
            $table->id();
        });
    }
};
