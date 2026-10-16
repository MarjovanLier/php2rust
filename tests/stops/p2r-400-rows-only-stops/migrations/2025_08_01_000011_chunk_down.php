<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Schema;

// Every call is a DB::table chain, but chunk() calls down(), which changes a column.
return new class extends Migration
{
    public function up(): void
    {
        DB::table('axles')->orderBy('id')->chunk(100, [$this, 'down']);
    }

    public function down(): void
    {
        Schema::table('axles', function (Blueprint $table) {
            $table->dropColumn('name');
        });
    }
};
