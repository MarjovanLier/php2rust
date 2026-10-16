<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Schema;

// PHP 8.5 allows a closure in a property default, and chunk() runs it.
return new class extends Migration
{
    private $drop = static function (): void {
        Schema::table('cams', static function (Blueprint $table) {
            $table->dropColumn('name');
        });
    };

    public function up(): void
    {
        DB::table('cams')->orderBy('id')->chunk(100, $this->drop);
    }
};
