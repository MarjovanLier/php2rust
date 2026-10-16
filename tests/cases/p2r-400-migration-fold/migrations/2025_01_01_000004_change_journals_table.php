<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
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
        if (! Schema::hasTable('codes')) {
            Schema::create('codes', function (Blueprint $table) {
                $table->id();
                $table->string('code');
            });
        }
    }

    public function down(): void
    {
        Schema::dropIfExists('codes');
    }
};
