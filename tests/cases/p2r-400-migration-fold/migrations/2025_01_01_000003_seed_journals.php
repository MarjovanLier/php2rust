<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Support\Facades\DB;

// It changes rows only, so the tool skips it.
return new class extends Migration
{
    public function up(): void
    {
        $now = now();
        foreach ([1, 2] as $number) {
            DB::table('journals')->insert(['code' => 'J' . $number, 'created_at' => $now]);
        }
        $first = DB::table('journals')->where('code', 'J1')->first();
        if (! $first) {
            return;
        }
        $this->note();
    }

    private function note(): void
    {
        DB::table('journals')->where('code', 'J1')->update(['note' => DB::raw("'first'")]);
    }

    public function down(): void
    {
        DB::table('journals')->delete();
    }
};
