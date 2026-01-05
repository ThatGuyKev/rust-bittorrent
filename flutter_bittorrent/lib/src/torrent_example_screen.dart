import 'package:flutter/material.dart';
import 'package:flutter_bittorrent/src/ipc/torrent_events.dart';
import 'package:flutter_bittorrent/src/ipc/torrent_notifications.dart';
import 'package:file_picker/file_picker.dart';
import 'dart:async';

/// Example screen demonstrating IPC with Rust backend
class TorrentExampleScreen extends StatefulWidget {
  const TorrentExampleScreen({super.key});

  static const routeName = '/torrent_example';

  @override
  State<TorrentExampleScreen> createState() => _TorrentExampleScreenState();
}

class _TorrentExampleScreenState extends State<TorrentExampleScreen> {
  String _status = 'Ready';
  String _torrentId = '';
  double _progress = 0.0;
  int _downloadSpeed = 0;
  final List<String> _logs = [];
  final List<StreamSubscription> _subscriptions = [];
  final TextEditingController _magnetController = TextEditingController();
  String _selectedFilePath = '';

  @override
  void initState() {
    super.initState();
    _setupNotificationListeners();
  }

  @override
  void dispose() {
    // Clean up subscriptions
    for (final sub in _subscriptions) {
      sub.cancel();
    }
    _magnetController.dispose();
    super.dispose();
  }

  void _setupNotificationListeners() {
    // Listen to progress notifications
    final progressSub = TorrentNotifications.onProgress((notification) {
      setState(() {
        _progress = notification.progress;
        _downloadSpeed = notification.downloadSpeed.toInt();
        _addLog('Progress: ${(_progress * 100).toStringAsFixed(1)}% - '
            'Speed: ${(_downloadSpeed / 1024 / 1024).toStringAsFixed(2)} MB/s');
      });
    });
    _subscriptions.add(progressSub);

    // Listen to completion notifications
    final completedSub = TorrentNotifications.onCompleted((notification) {
      setState(() {
        _status = 'Completed';
        _progress = 1.0;
        _addLog('✓ Download completed: ${notification.name}');
      });
    });
    _subscriptions.add(completedSub);

    // Listen to error notifications
    final errorSub = TorrentNotifications.onError((notification) {
      setState(() {
        _status = 'Error';
        _addLog('✗ Error: ${notification.errorMessage}');
      });
    });
    _subscriptions.add(errorSub);
  }

  void _addLog(String message) {
    setState(() {
      _logs.insert(0,
          '${DateTime.now().toIso8601String().substring(11, 19)} - $message');
      if (_logs.length > 20) {
        _logs.removeLast();
      }
    });
  }

  Future<void> _startTorrent() async {
    // Determine the torrent source
    String torrentPath;

    if (_magnetController.text.trim().isNotEmpty) {
      torrentPath = _magnetController.text.trim();
      _addLog('Using magnet link');
    } else if (_selectedFilePath.isNotEmpty) {
      torrentPath = _selectedFilePath;
      _addLog('Using torrent file: $_selectedFilePath');
    } else {
      _addLog('⚠ Please select a torrent file or enter a magnet link');
      return;
    }

    setState(() {
      _status = 'Starting...';
    });

    try {
      _addLog('Dispatching StartTorrent event...');

      // Call the event using the direct API
      final response = await TorrentEvents.startTorrent(
        torrentPath: torrentPath,
        downloadDir: '/downloads',
      );

      setState(() {
        _torrentId = response.torrentId;
        _status = 'Downloading';
        _addLog('✓ Torrent started: ${response.name}');
        _addLog('  ID: ${response.torrentId}');
        _addLog(
            '  Size: ${(response.totalSize.toInt() / 1024 / 1024).toStringAsFixed(2)} MB');
      });
    } catch (e) {
      setState(() {
        _status = 'Error';
        _addLog('✗ Failed to start torrent: $e');
      });
    }
  }

  Future<void> _pickTorrentFile() async {
    try {
      FilePickerResult? result = await FilePicker.platform.pickFiles(
        type: FileType.custom,
        allowedExtensions: ['torrent'],
      );

      if (result != null && result.files.single.path != null) {
        setState(() {
          _selectedFilePath = result.files.single.path!;
          _magnetController.clear(); // Clear magnet link if file selected
        });
        _addLog('✓ Selected file: ${result.files.single.name}');
      }
    } catch (e) {
      _addLog('✗ Failed to pick file: $e');
    }
  }

  Future<void> _getTorrentStatus() async {
    if (_torrentId.isEmpty) {
      _addLog('⚠ No active torrent');
      return;
    }

    try {
      _addLog('Fetching torrent status...');

      final response = await TorrentEvents.getTorrentStatus(
        torrentId: _torrentId,
      );

      _addLog('Status: ${response.status}');
      _addLog('Progress: ${(response.progress * 100).toStringAsFixed(1)}%');
      _addLog(
          'Downloaded: ${(response.downloaded.toInt() / 1024 / 1024).toStringAsFixed(2)} MB');
    } catch (e) {
      _addLog('✗ Failed to get status: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('BitTorrent Client'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // Torrent source selection
            Card(
              child: Padding(
                padding: const EdgeInsets.all(16.0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'Add Torrent',
                      style: Theme.of(context).textTheme.titleLarge,
                    ),
                    const SizedBox(height: 16),

                    // File picker button
                    Row(
                      children: [
                        Expanded(
                          child: ElevatedButton.icon(
                            onPressed: _pickTorrentFile,
                            icon: const Icon(Icons.folder_open),
                            label: Text(_selectedFilePath.isEmpty
                                ? 'Select .torrent File'
                                : 'File: ${_selectedFilePath.split('/').last}'),
                          ),
                        ),
                        if (_selectedFilePath.isNotEmpty)
                          IconButton(
                            onPressed: () {
                              setState(() {
                                _selectedFilePath = '';
                              });
                            },
                            icon: const Icon(Icons.clear),
                            tooltip: 'Clear selection',
                          ),
                      ],
                    ),
                    const SizedBox(height: 12),

                    // OR divider
                    Row(
                      children: [
                        Expanded(child: Divider()),
                        Padding(
                          padding: const EdgeInsets.symmetric(horizontal: 8.0),
                          child: Text('OR',
                              style: Theme.of(context).textTheme.bodySmall),
                        ),
                        Expanded(child: Divider()),
                      ],
                    ),
                    const SizedBox(height: 12),

                    // Magnet link input
                    TextField(
                      controller: _magnetController,
                      decoration: InputDecoration(
                        labelText: 'Magnet Link',
                        hintText: 'magnet:?xt=urn:btih:...',
                        prefixIcon: const Icon(Icons.link),
                        border: const OutlineInputBorder(),
                        suffixIcon: _magnetController.text.isNotEmpty
                            ? IconButton(
                                icon: const Icon(Icons.clear),
                                onPressed: () {
                                  setState(() {
                                    _magnetController.clear();
                                  });
                                },
                              )
                            : null,
                      ),
                      onChanged: (value) {
                        if (value.isNotEmpty) {
                          setState(() {
                            _selectedFilePath =
                                ''; // Clear file if magnet entered
                          });
                        }
                      },
                      maxLines: 2,
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 16),

            // Status card
            Card(
              child: Padding(
                padding: const EdgeInsets.all(16.0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'Status: $_status',
                      style: Theme.of(context).textTheme.titleLarge,
                    ),
                    const SizedBox(height: 8),
                    if (_torrentId.isNotEmpty) ...[
                      Text('Torrent ID: $_torrentId'),
                      const SizedBox(height: 8),
                    ],
                    LinearProgressIndicator(value: _progress),
                    const SizedBox(height: 4),
                    Text('${(_progress * 100).toStringAsFixed(1)}%'),
                    if (_downloadSpeed > 0) ...[
                      const SizedBox(height: 4),
                      Text(
                          'Speed: ${(_downloadSpeed / 1024 / 1024).toStringAsFixed(2)} MB/s'),
                    ],
                  ],
                ),
              ),
            ),
            const SizedBox(height: 16),

            // Action buttons
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: [
                ElevatedButton.icon(
                  onPressed: _startTorrent,
                  icon: const Icon(Icons.play_arrow),
                  label: const Text('Start Torrent'),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Colors.green,
                    foregroundColor: Colors.white,
                  ),
                ),
                ElevatedButton.icon(
                  onPressed: _getTorrentStatus,
                  icon: const Icon(Icons.info),
                  label: const Text('Get Status'),
                ),
              ],
            ),
            const SizedBox(height: 16),

            // Logs
            Expanded(
              child: Card(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Padding(
                      padding: const EdgeInsets.all(16.0),
                      child: Text(
                        'Event Log',
                        style: Theme.of(context).textTheme.titleMedium,
                      ),
                    ),
                    const Divider(height: 1),
                    Expanded(
                      child: ListView.builder(
                        itemCount: _logs.length,
                        itemBuilder: (context, index) {
                          return Padding(
                            padding: const EdgeInsets.symmetric(
                              horizontal: 16.0,
                              vertical: 4.0,
                            ),
                            child: Text(
                              _logs[index],
                              style: const TextStyle(
                                fontFamily: 'monospace',
                                fontSize: 12,
                              ),
                            ),
                          );
                        },
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
