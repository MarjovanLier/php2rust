<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('levers', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        // A method named by an expression may be getConnection().
        DB::table('levers')->{'getConnection'}()->statement('ALTER TABLE levers DROP COLUMN name');
    }
};
