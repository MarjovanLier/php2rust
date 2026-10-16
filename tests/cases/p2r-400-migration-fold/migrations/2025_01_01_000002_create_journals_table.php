<?php

declare(strict_types=1);

use App\Administration;
use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('journals', function (Blueprint $table): void {
            $table->id();
            $table->foreignIdFor(Administration::class)->constrained();
            // After constrained(), nullable() belongs to the foreign key.
            $table->foreignId('owner_id')->constrained('users')->nullable();
            $table->string('code')->unique();
            $table->string('note')->nullable();
            $table->rememberToken();
            $table->foreign('code')->references('code')->on('codes')->cascadeOnDelete();
            $table->unique(['administration_id', 'code']);
        });
        DB::table('journals')->insert(['code' => 'J0']);
    }

    public function down(): void
    {
        Schema::dropIfExists('journals');
    }
};
