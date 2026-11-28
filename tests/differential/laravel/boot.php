<?php

declare(strict_types=1);

// The PHP side of a Laravel subject runs after this file, which the harness gives with
// auto_prepend_file; php2rust never reads it (P2R-606). It boots Eloquent on an in-memory
// SQLite database, sets the facade root that Schema::create needs, and runs the subject's
// migrations, which PHP2RUST_MIGRATIONS names, in the byte order of their names without
// .php, as Laravel and P2R-400 do.

require __DIR__ . '/vendor/autoload.php';

$capsule = new Illuminate\Database\Capsule\Manager();
$capsule->addConnection(['driver' => 'sqlite', 'database' => ':memory:']);
$capsule->setAsGlobal();
$capsule->bootEloquent();

$app = new Illuminate\Container\Container();
$app->instance('db', $capsule->getDatabaseManager());
$app->bind('db.schema', static fn () => $capsule->getConnection()->getSchemaBuilder());
Illuminate\Support\Facades\Facade::setFacadeApplication($app);

$migrations = getenv('PHP2RUST_MIGRATIONS');
if (is_string($migrations) && $migrations !== '') {
    $files = array_filter(
        glob($migrations . '/*_*.php') ?: [],
        static fn (string $file): bool => is_file($file),
    );
    usort($files, static fn (string $a, string $b): int => strcmp(basename($a, '.php'), basename($b, '.php')));
    foreach ($files as $file) {
        (require $file)->up();
    }
}
