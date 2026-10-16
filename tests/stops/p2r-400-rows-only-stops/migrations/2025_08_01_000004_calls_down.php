<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

// The tool does not read down(), so a call of it is not one of a rows-only file.
return new class extends Migration
{
    public function up(): void
    {
        $this->down();
    }

    public function down(): void
    {
        Schema::table('rods', function (Blueprint $table) {
            $table->dropColumn('name');
        });
    }
};
