<?php
header('Content-Type: text/html; charset=UTF-8');

// Zugangsdaten
$db_server = 'database-5019451838.webspace-host.com';
$db_benutzer = 'dbu2916835';
$db_passwort = 'My_Rss123!';
$db_name = 'dbs15220279';

// Verbindungsaufbau
try {
    $dbh = new PDO("mysql:host=$db_server;dbname=$db_name", $db_benutzer, $db_passwort);
    //echo 'Verbindung zur Datenbank hergestellt.';
} catch (PDOException $e) {
    die('Fehler bei der Verbindung zur Datenbank: ' . $e->getMessage());
}

// Tabelle
//rss(id autoincrement, name, url, active)

// Return rss table as JSON when called with POST action=get_channels
if ($_SERVER['REQUEST_METHOD'] === 'POST' && ($_POST['action'] ?? '') === 'get_channels') {
    try {
        $stmt = $dbh->prepare("SELECT * FROM rss");
        $stmt->execute();
        $rss_data = $stmt->fetchAll(PDO::FETCH_ASSOC);

        header('Content-Type: application/json; charset=UTF-8');
        echo json_encode($rss_data, JSON_UNESCAPED_UNICODE);
    } catch (PDOException $e) {
        http_response_code(500);
        header('Content-Type: application/json; charset=UTF-8');
        echo json_encode([], JSON_UNESCAPED_UNICODE);
    }
    exit;
}

// Handle form submission for adding new RSS feed
if ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_POST['name'])) {
    $name = $_POST['name'] ?? '';
    $url = $_POST['rss_url'] ?? '';
    $active = isset($_POST['active']) ? 1 : 0;
    
    if (!empty($name) && !empty($url)) {
        $stmt = $dbh->prepare("INSERT INTO rss (name, url, active) VALUES (:name, :url, :active)");
        $stmt->bindParam(':name', $name);
        $stmt->bindParam(':url', $url);
        $stmt->bindParam(':active', $active, PDO::PARAM_INT);

        if ($stmt->execute()) {
            $success_message = "RSS feed added successfully!";
        } else {
            $error_message = "Error adding RSS feed.";
        }
    } else {
        $error_message = "Name and URL are required fields.";
    }
}

// Query all data from rss table
$stmt = $dbh->prepare("SELECT * FROM rss");
$stmt->execute();
$rss_data = $stmt->fetchAll(PDO::FETCH_ASSOC);
?>

<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>RSS Feed Manager</title>
</head>

<body>
    <h1>RSS Feed Manager</h1>

    <?php if (isset($success_message)): ?>
        <p style="color: green;"><?php echo htmlspecialchars($success_message); ?></p>
    <?php endif; ?>

    <?php if (isset($error_message)): ?>
        <p style="color: red;"><?php echo htmlspecialchars($error_message); ?></p>
    <?php endif; ?>

    <h2>Add New RSS Feed</h2>
    <form method="POST" accept-charset="UTF-8">
        <table>
            <tr>
                <td><label for="name">Name:</label></td>
                <td><input type="text" id="name" name="name" required></td>
            </tr>
            <tr>
                <td><label for="rss_url">URL:</label></td>
                <td><input type="text" id="rss_url" name="rss_url" required placeholder="https://example.com/feed.xml"></td>
            </tr>
            <tr>
                <td><label for="active">Active:</label></td>
                <td><input type="checkbox" id="active" name="active" checked></td>
            </tr>
            <tr>
                <td colspan="2">
                    <button type="submit">Add RSS Feed</button>
                </td>
            </tr>
        </table>
    </form>

    <h2>RSS Feeds</h2>
    <table border="1" cellpadding="10" cellspacing="0">
        <tr>
            <th>ID</th>
            <th>Name</th>
            <th>URL</th>
            <th>Active</th>
        </tr>
        <?php
        foreach ($rss_data as $row) {
            echo "<tr>";
            echo "<td>" . htmlspecialchars($row['id']) . "</td>";
            echo "<td>" . htmlspecialchars($row['name']) . "</td>";
            echo "<td>" . htmlspecialchars($row['url']) . "</td>";
            echo "<td>" . htmlspecialchars($row['active']) . "</td>";
            echo "</tr>";
        }
        ?>
    </table>
</body>

</html>
