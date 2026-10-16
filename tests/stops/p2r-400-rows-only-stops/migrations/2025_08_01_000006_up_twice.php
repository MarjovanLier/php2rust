<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

// PHP rejects a second method of one name, so Laravel stops at this file.
return new class extends Migration
{
    public function up(): void
    {
        Schema::table('belts', function (Blueprint $table) {
            $table->string('size');
        });
    }

    public function UP(): void
    {
    }
};
