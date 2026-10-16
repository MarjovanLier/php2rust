<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

// Laravel runs this when it requires the file, before any up().
Schema::table('wheels', function (Blueprint $table) {
    $table->dropColumn('name');
});

return new class extends Migration
{
    public function up(): void
    {
    }
};
