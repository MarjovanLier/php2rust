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
        Schema::table('journals', static function (Blueprint $table): void {
            $table->string('note')->nullable(false)->change();
            $table->dropColumn(['remember_token']);
            $table->integer('lines')->default(0)->after('code');
        });
        // Its callback changes rows only, so the statement is skipped.
        DB::table('journals')->orderBy('id')->chunk(10, function ($rows) {
            DB::table('journals')->where('lines', count($rows))->update(['lines' => 0]);
        });
        if (! Schema::hasTable('codes')) {
            Schema::create('codes', function (Blueprint $table) {
                // Laravel casts the key id to int, so it reads as i64.
                $table->uuid('id')->primary();
                $table->string('code');
            });
        }
    }

    public function down(): void
    {
        Schema::dropIfExists('codes');
    }
};
