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
        Schema::create('hinges', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
        // chunk() calls the method the array names, which the tool does not read.
        DB::table('hinges')->orderBy('id')->chunk(100, [$this, 'alter']);
        Schema::create('axles', function (Blueprint $table) {
            $table->id();
            $table->string('name');
        });
    }

    public function alter(): void
    {
        Schema::table('hinges', function (Blueprint $table) {
            $table->dropColumn('name');
        });
    }
};
