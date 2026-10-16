
<?php

// The blank line before <?php only prints, so the file is read. PHP refuses such
// text before declare(strict_types=1), so this file has no declare.

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    protected $connection = 'mysql_migration';

    public function up(): void
    {
        Schema::create('administrations', function (Blueprint $table) {
            $table->id();
            $table->uuid();
            $table->string('name', 120);
            $table->timestamps();
            $table->softDeletes();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('administrations');
    }
};
