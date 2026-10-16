<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('flags', function (Blueprint $table) {
            $table->id();
            $table->boolean('enabled');
            $table->boolean('visible')->nullable();
            $table->boolean('counted');
            $table->integer('weight');
            $table->string('code');
            $table->timestamp('opened_at');
            $table->decimal('ratio', 8, 2);
        });
    }
};
