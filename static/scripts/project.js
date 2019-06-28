$('.formInput input, .formInput select, .formInput textarea').each(function() {
	if ($(this).val() !== '') {
		$(`#${this.id} + label`).animate({
			'fontSize': '0.8rem',
			'top': '-0.7rem',
			'padding': '0.25rem'
		}, 80);
	}
	$(this).focusin(() => {
		$(`#${this.id} + label`).animate({
			'fontSize': '0.8rem',
			'top': '-0.7rem',
			'padding': '0.25rem'
		}, 80);
	});
	$(this).focusout(function() {
		if ($(this).val() === '') {
			$(`#${this.id} + label`).animate({
				'fontSize': '1rem',
				'top': '1rem',
				'padding': 0
			}, 80);
		}
	});
	$(this).keyup(function () {
		if (this.id === 'password' && $(this).val().length > 0 && $(this).val().length < 8) {
			$('#password').parent().attr('class', 'formInput error');
			$('#password ~ .helper-text').text('This has to be at least 8 characters');
		} else if (this.id === 'projectAbstract' && $(this).val().split(' ').length > 120) {
			$(this).parent().attr('class', 'formInput error');
			$('#projectAbstract ~ .helper-text').text('The abstract must be lesser than 120 words long');
		} else {
			$(this).parent().attr('class', 'formInput');
			if (this.id !== 'class') {
				$(`#${this.id} ~ .helper-text`).empty();
			}
		}
	});
});

$('#edit').click(() => {
	$('.overlay').show();
	$('#name').val(user_details.name);
	$('#phone').val(user_details.phone);
	$('#editProfile .formInput input').each(function() {
		if ($(this).val() !== '') {
			$(`#${this.id} + label`).animate({
				'fontSize': '0.8rem',
				'top': '-0.7rem',
				'padding': '0.25rem'
			}, 80);
		}
	});
	$('#editProfile').show('slow');
});

$('#editProjectButton').click(() => {
	$('.overlay').show();
	$('#projectTitle').val(project_details.project_title);
	$('#projectAbstract').val(project_details.project_abstract.length === 0 ?
		project_details.division === 'science' ?
			'This must be a short description of your project. You can try to enumerate the key features of your project, or the problems it tries to solve. You can link to attachments if you want. Please note that you can easily edit the project title and abstract later.' :
			'(Optional) You can give a short description of your project here.' :
		project_details.project_abstract);
	$('#editProject .formInput input, #editProject .formInput textarea').each(function() {
		if ($(this).val() !== '') {
			$(`#${this.id} + label`).animate({
				'fontSize': '0.8rem',
				'top': '-0.7rem',
				'padding': '0.25rem'
			}, 80);
		}
	});
	$('#editProject').show('slow');
});

$('.overlay, #editProfile .textButton, #editProject .textButton, #addTeamMember .textButton').click(() => {
	$('.dialog').hide('slow', () => $('.overlay').hide());
});

$('#editProfile form').submit(e => {
	e.preventDefault();
	if ($('#name').val().length === 0) {
		$('#name').parent().attr('class', 'formInput error');
		$('#name ~ .helper-text').text('Enter your name');
		showToast('Errors in form');
		return;
	}
	if ($('#phone').is(':invalid')) {
		$('#phone').parent().attr('class', 'formInput error');
		$('#phone ~ .helper-text').text('Enter a valid phone number or none at all');
		showToast('Errors in form');
		return;
	}
	if ($('#password').val().length > 0 && $('#password').val().length < 8) {
		$('#password').parent().attr('class', 'formInput error');
		$('#password ~ .helper-text').text('This has to be at least 8 characters');
		showToast('Errors in form');
		return;
	}
	showToast('Please wait...', 10000);
	$.ajax({
		type: 'POST',
		url: '/update_profile',
		data: {
			name: $('#name').val(),
			phone: $('#phone').val().length === 0 ? undefined : $('#phone').val(),
			new_password: $('#password').val()
		},
		success: result => {
			console.log(result);
			showToast(result, 10000);
			if (result === 'success') window.location.reload();
		}
	});
});

$('#editProject form').submit(e => {
	e.preventDefault();
	if ($('#projectTitle').val().length === 0) {
		$('#projectTitle').parent().attr('class', 'formInput error');
		$('#projectTitle ~ .helper-text').text('Enter a project title');
		showToast('Errors in form');
		return;
	}
	if (project_details.division === 'science' && $('#projectAbstract').val().split(' ').length < 50) {
		$('#projectAbstract').parent().attr('class', 'formInput error');
		$('#projectAbstract ~ .helper-text').text('The project abstract must be at least 50 words long');
		showToast('Errors in form');
		return;
	}
	if ($('#projectAbstract').val().split(' ').length > 120) {
		$('#projectAbstract').parent().attr('class', 'formInput error');
		$('#projectAbstract ~ .helper-text').text('The abstract must be lesser than 120 words long');
		showToast('Errors in form');
		return;
	}
	showToast('Please wait...', 10000);
	$.ajax({
		type: 'POST',
		url: '/update_project',
		data: {
			project_title: $('#projectTitle').val(),
			project_abstract: $('#projectAbstract').val()
		},
		success: result => {
			console.log(result);
			showToast(result, 10000);
			if (result === 'success') window.location.reload();
		}
	});
});

$('#addTeamMember form').submit(e => {
	e.preventDefault();
	if ($('#newMemberName').val().length === 0) {
		$('#newMemberName').parent().attr('class', 'formInput error');
		$('#newMemberName ~ .helper-text').text('Enter the name');
		showToast('Enter the name');
		return;
	}
	if ($('#newMemberEmail').val().length === 0 || $('#newMemberEmail').is(':invalid')) {
		$('#newMemberEmail').parent().attr('class', 'formInput error');
		$('#newMemberEmail ~ .helper-text').text('Enter a valid email address');
		showToast('Enter a valid email address');
		return;
	}
	if ($('#newMemberPassword').val().length < 8) {
		$('#newMemberPasswordpassword').parent().attr('class', 'formInput error');
		$('#newMemberPassword ~ .helper-text').text('This has to be at least 8 characters');
		showToast('The password has to be at least 8 characters');
		return;
	}
	if ($('#newMemberSection').val().length === 0) {
		$('#newMemberSection').parent().attr('class', 'formInput error');
		$('#newMemberSection ~ .helper-text').text('Enter the section');
		showToast('Enter the section');
		return;
	}
	showToast('Please wait...', 10000);
	$.ajax({
		type: 'POST',
		url: '/add_team_member',
		data: {
			name: $('#newMemberName').val(),
			email: $('#newMemberEmail').val(),
			password: $('#newMemberPassword').val(),
			grade: $('#newMemberClass').val(),
			section: $('#newMemberSection').val()
		},
		success: result => {
			console.log(result);
			showToast(result, 10000);
			if (result === 'success') window.location.reload();
		}
	});
});

$('#signoutButton').click(() => {
	$.ajax({
		type: 'POST',
		url: '/signout',
		success: (result) => {
			console.log(result);
			showToast(result, 10000);
			if (result === 'success') window.location.href = '/';
		}
	});
});
