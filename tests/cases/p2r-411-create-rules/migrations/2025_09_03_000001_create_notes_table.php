<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('notes', function (Blueprint $table) {
            $table->id();
            $table->string('title');
            $table->text('body')->nullable();
            $table->integer('rank');
            $table->string('status')->nullable()->default('new');
            $table->timestamps();
        });
        Schema::create('stamps', function (Blueprint $table) {
            $table->id();
            $table->string('label');
            $table->timestamp('created_at');
            $table->timestamp('updated_at');
        });
    }
};
