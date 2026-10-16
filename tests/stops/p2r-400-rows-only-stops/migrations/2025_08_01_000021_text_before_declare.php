
<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

// PHP refuses text before declare(strict_types=1), so Laravel stops at this file.
return new class extends Migration
{
    public function up(): void
    {
        Schema::table('rivets', function (Blueprint $table) {
            $table->string('size');
        });
    }
};
