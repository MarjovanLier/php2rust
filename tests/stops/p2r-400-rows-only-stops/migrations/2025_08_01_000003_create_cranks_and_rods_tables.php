<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        // A named argument names the column crank_id, which the tool does not read.
        Schema::create('cranks', function (Blueprint $table) {
            $table->id(column: 'crank_id');
        });
        Schema::create('rods', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
    }
};
