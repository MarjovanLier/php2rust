<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

// Laravel asks shouldRun() first, so the tool cannot know whether up() runs.
return new class extends Migration
{
    public function shouldRun(): bool
    {
        return false;
    }

    public function up(): void
    {
        Schema::table('nuts', function (Blueprint $table) {
            $table->string('size');
        });
    }
};
