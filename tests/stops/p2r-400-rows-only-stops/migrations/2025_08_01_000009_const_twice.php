<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

// PHP rejects a second constant of one name, so Laravel stops at this file.
return new class extends Migration
{
    private const SIZE = 'size';

    private const SIZE = 'width';

    public function up(): void
    {
        Schema::table('springs', function (Blueprint $table) {
            $table->string('size');
        });
    }
};
