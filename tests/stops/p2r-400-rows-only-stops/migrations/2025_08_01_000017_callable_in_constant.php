<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Support\Facades\DB;

// PHP 8.5 allows a first-class callable in a constant, and when() runs it.
return new class extends Migration
{
    private const DROP = drop_gasket_names(...);

    public function up(): void
    {
        DB::table('gaskets')->when(true, self::DROP);
    }
};
