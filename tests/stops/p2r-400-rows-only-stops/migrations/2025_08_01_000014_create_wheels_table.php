<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        // A named argument that binds no column keeps the table known.
        Schema::create('wheels', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->timestamps(precision: 6);
        });
    }
};
