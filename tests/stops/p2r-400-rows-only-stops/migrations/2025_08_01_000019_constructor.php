<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

// PHP runs the constructor when Laravel requires the file.
return new class extends Migration
{
    public function __construct()
    {
        Schema::table('seals', function (Blueprint $table) {
            $table->dropColumn('name');
        });
    }

    public function up(): void
    {
        Schema::table('seals', function (Blueprint $table) {
            $table->string('size')->nullable();
        });
    }
};
